package main

import (
	"github.com/gin-contrib/cors"
	"github.com/gin-gonic/gin"
	"gitlab.com/resamvi/wasletztegesetz/resource"
)

func main() {
	r := gin.Default()
	r.Use(cors.Default())

	protokolle := resource.Fetch()

	r.GET("/", func(c *gin.Context) {
		c.JSON(200, "OK")
	})
	r.GET("/plenarprotokolle", func(c *gin.Context) {
		c.JSON(200, protokolle)
	})

	r.Run(":1337")
}

