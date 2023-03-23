package main

import (
	"net/http"
	"time"

	"github.com/gin-gonic/gin"
)

func hello(c *gin.Context) {
	time.Sleep(500 * time.Millisecond) // Simulate some processing time
	c.String(http.StatusOK, "Hello, World!")
}

func main() {
	r := gin.Default()
	r.GET("/", hello)
	r.Run(":80")
}
