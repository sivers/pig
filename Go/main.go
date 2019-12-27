package main

// QUESTIONS
//
// #1: Is all of this Unmarshal-to-obj necessary?
// PostgreSQL is already returning JSON string.
// Send that string to HTTP response directly, instead.
//
// #2: Similar to #1, PostgreSQL is already returning HTTP status code.
// Send that status code to HTTP response instead of Gin constants.
//
// #3: Instead of new and unique function for each route,
// one common function that just takes a string, like the Ruby example.
// Then middleware to handle parameter requirements before that.
//
// Any answers? Please email derek@sivers.org

import (
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"regexp"
	"strings"

	"github.com/gin-gonic/gin"
)

func init() {
	err := migrateInternal()
	if err != nil {
		panic(err)
	}
}

func main() {

	SetSchema("pig")

	router := gin.Default()

	router.GET("/", reqAuthUser, getPerson)

	router.GET("/person/:id", reqAuthUser, getPersonByID)
	router.PATCH("/person", reqAuthUser, patchPerson)

	things := router.Group("/things", reqAuthUser)
	{
		things.GET("", getThings)
		things.POST("", addThings)
	}

	thing := router.Group("/thing", reqAuthUser)
	{
		thing.GET("/:id", getThingsByID)
		thing.PATCH("/:id", patchThings)
		thing.DELETE("/:id", deleteThings)
	}

	router.Run()
}

// reqAuthUser returns middleware which requires authenticated user for request.
func reqAuthUser(ctx *gin.Context) {
	r, _ := regexp.Compile(`\A[a-z]{4}\z`)

	apikey := strings.TrimSpace(ctx.GetHeader("apikey"))

	if !r.MatchString(apikey) {
		ctx.AbortWithStatusJSON(http.StatusUnauthorized,
			gin.H{"error": "needs apikey header"})
		return
	}

	code, jsString, err := Q("apikey_get", []string{apikey})
	if err != nil || *code == 404 {
		ctx.AbortWithStatusJSON(http.StatusUnauthorized,
			gin.H{"error": "wrong apikey"})
		return
	}

	var obj map[string]int
	json.Unmarshal([]byte(jsString), &obj)

	ctx.Set("id", fmt.Sprint(obj["person_id"]))
	ctx.Next()
}

// getPerson
func getPerson(ctx *gin.Context) {
	code, jsString, err := Q("people_get", []string{})
	if err != nil {
		log.Println(err)
	}

	var obj interface{}
	json.Unmarshal([]byte(jsString), &obj)
	ctx.JSON(*code, obj)
}

// getPersonByID
func getPersonByID(ctx *gin.Context) {
	code, obj, err := Q("person_get", []string{ctx.Param("id")})
	if err != nil {
		log.Println(err)
	}

	var res interface{}
	json.Unmarshal([]byte(obj), &res)
	ctx.JSON(*code, res)
}

// patchPerson
func patchPerson(ctx *gin.Context) {
	name := ctx.PostForm("name")
	if name == "" {
		ctx.JSON(http.StatusPreconditionFailed,
			gin.H{"error": "missing name"})
		return
	}

	code, obj, err := Q("person_update", []string{fmt.Sprint(ctx.MustGet("id")), name})
	if err != nil {
		log.Println(err)
	}

	var res interface{}
	json.Unmarshal([]byte(obj), &res)
	ctx.JSON(*code, res)
}

// getThings
func getThings(ctx *gin.Context) {
	code, jsString, err := Q("things_get", []string{fmt.Sprint(ctx.MustGet("id"))})
	if err != nil {
		log.Println(err)
	}

	var obj interface{}
	json.Unmarshal([]byte(jsString), &obj)
	ctx.JSON(*code, obj)
}

// getThingsByID
func getThingsByID(ctx *gin.Context) {
	code, obj, err := Q("thing_get", []string{fmt.Sprint(ctx.MustGet("id")), ctx.Param("id")})
	if err != nil {
		log.Println(err)
		ctx.JSON(http.StatusInternalServerError,
			gin.H{"error": "internal server error"})
		return
	}

	var res interface{}
	json.Unmarshal([]byte(obj), &res)
	ctx.JSON(*code, res)
}

// patchThings
func patchThings(ctx *gin.Context) {
	name := ctx.PostForm("name")
	if name == "" {
		ctx.JSON(http.StatusPreconditionFailed,
			gin.H{"error": "missing name"})
		return
	}

	code, obj, err := Q("thing_update", []string{fmt.Sprint(ctx.MustGet("id")), ctx.Param("id"), name})
	if err != nil {
		log.Println(err)
	}

	var res interface{}
	json.Unmarshal([]byte(obj), &res)
	ctx.JSON(*code, res)
}

// addThings
func addThings(ctx *gin.Context) {
	name := ctx.PostForm("name")
	if name == "" {
		ctx.JSON(http.StatusPreconditionFailed,
			gin.H{"error": "missing name"})
		return
	}

	code, obj, err := Q("thing_add", []string{fmt.Sprint(ctx.MustGet("id")), name})
	if err != nil {
		log.Println(err)
	}

	var res interface{}
	json.Unmarshal([]byte(obj), &res)
	ctx.JSON(*code, res)
}

// deleteThings
func deleteThings(ctx *gin.Context) {
	code, obj, err := Q("thing_delete", []string{fmt.Sprint(ctx.MustGet("id")), ctx.Param("id")})
	if err != nil {
		log.Println(err)
		ctx.JSON(http.StatusInternalServerError,
			gin.H{"error": "internal server error"})
		return
	}

	var res interface{}
	json.Unmarshal([]byte(obj), &res)
	ctx.JSON(*code, res)
}
