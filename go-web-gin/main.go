package main

import (
	"encoding/json"
	"io/ioutil"
	"net/http"

	"github.com/gin-gonic/gin"
)

func main() {
	r := gin.Default()
	r.POST("/call", callHandler)
	r.Run(":8086")
}

func callHandler(c *gin.Context) {
	var json HttpReq
	if err := c.ShouldBindJSON(&json); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	info, err := makeHttpRequest(json.Url)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{
			"error": "failed to retrieve info",
			"root":  err.Error(),
		})
	}

	c.JSON(http.StatusOK, info)
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
