package main

import (
	"encoding/json"
	"fmt"
	"io"
	"io/ioutil"
	"log"
	"net/http"
)

func main() {
	http.HandleFunc("/call", handler)
	if err := http.ListenAndServe(":8081", nil); err != nil {
		log.Fatal(err)
	}
}

func handler(writer http.ResponseWriter, req *http.Request) {
	if req.Method != "POST" {
		fmt.Printf("Unacceptable method: %s\n", req.Method)
		http.Error(writer, "Only POST supported", http.StatusMethodNotAllowed)
		return
	}

	// Doesn't check for appropriate Content-Type header
	body := req.Body
	defer body.Close()
	bytes, err := readBody(req.Body)
	if err != nil {
		fmt.Printf("Unable to read request body: %s\n", err.Error())
		http.Error(writer, err.Error(), http.StatusBadRequest)
		return
	}

	var httpReq HttpReq
	if err := json.Unmarshal(bytes, &httpReq); err != nil {
		fmt.Printf("Unable to deserialize body: %s\n%s\n", err.Error(), string(bytes))
		http.Error(writer, err.Error(), http.StatusBadRequest)
		return
	}

	info, err := makeHttpRequest(httpReq.Url)
	if err != nil {
		http.Error(writer, err.Error(), http.StatusInternalServerError)
	}

	infoJson, err := json.Marshal(info)
	if err != nil {
		http.Error(writer, err.Error(), http.StatusInternalServerError)
	}

	writer.Write(infoJson)
}

func readBody(body io.ReadCloser) ([]byte, error) {
	defer body.Close()

	buff := make([]byte, 1024)
	bytes := []byte{}
	for {
		n, err := body.Read(buff)
		if err == io.EOF {
			if n > 0 {
				bytes = append(bytes, buff[:n]...)
			}
			break
		}
		if err != nil {
			return []byte{}, err
		}

		bytes = append(bytes, buff...)
	}

	return bytes, nil
}

func makeHttpRequest(url string) (*Info, error) {
	resp, err := http.Get(url)
	if err != nil {
		return nil, err
	}

	defer resp.Body.Close()
	body, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		return nil, err
	}

	var info Info
	if err = json.Unmarshal(body, &info); err != nil {
		return nil, err
	}

	return &info, nil
}

type HttpReq struct {
	Url string `json:"url"`
}

type Info struct {
	Id     uint   `json:"id"`
	Status string `json:"status"`
	Data   *Data  `json:"data"`
}

type Data struct {
	Requestors []string `json:"requestors"`
}
