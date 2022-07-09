package main

import (
	"encoding/json"
	"fmt"
	"io"
	"io/ioutil"
	"log"
	"net/http"
	"strings"
)

func main() {
	http.HandleFunc("/call", handler)
	if err := http.ListenAndServe(":8081", nil); err != nil {
		log.Fatal(err)
	}
}

func handler(writer http.ResponseWriter, req *http.Request) {
	fmt.Println("Handling request")
	if req.Method != "POST" {
		fmt.Printf("Unacceptable method: %s\n", req.Method)
		http.Error(writer, "Only POST supported", http.StatusMethodNotAllowed)
		return
	}

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

	if err := makeHttpRequest(httpReq.Url); err != nil {
		http.Error(writer, err.Error(), http.StatusInternalServerError)
	}
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

type HttpReq struct {
	Url string `json:"url"`
}

func makeHttpRequest(url string) error {
	resp, err := http.Get(url)
	if err != nil {
		return err
	}

	defer resp.Body.Close()
	body, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		return err
	}

	var info Info = newInfo()
	if err = json.Unmarshal(body, &info); err != nil {
		fmt.Printf("Error: %s\n", err)
	} else {
		fmt.Printf("%s\n", info.String())
	}

	return nil
}

func newInfo() Info {
	return Info{
		Id:     0,
		Status: "",
		Data:   &Data{Requestors: []string{}},
	}
}

type Info struct {
	Id     uint   `json:"id"`
	Status string `json:"status"`
	Data   *Data  `json:"data"`
}

func (i *Info) String() string {
	return "{" +
		`"Id": ` + fmt.Sprintf("%d", i.Id) + ", " +
		`"Status": "` + i.Status + `"` + ", " +
		`"Data": ` + i.Data.String() +
		"}"
}

type Data struct {
	Requestors []string `json:"requestors"`
}

func (d *Data) String() string {
	formatted := []string{}
	for _, s := range d.Requestors {
		formatted = append(formatted, fmt.Sprintf(`"%s"`, s))
	}
	return fmt.Sprintf(`{"Requestors": [%s]}`, strings.Join(formatted, ","))
}
