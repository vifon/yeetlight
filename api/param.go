package api

import (
	"errors"
	"fmt"
	"net/http"
	"strconv"
)

type Param interface {
	Get(r *http.Request) (interface{}, error)
}

type ConstParam struct {
	Value interface{}
}
func (c ConstParam) Get(*http.Request) (interface{}, error) {
	return c.Value, nil
}

type GetParam struct {
	param string
}
func (c GetParam) Get(r *http.Request) (interface{}, error) {
	value := r.URL.Query().Get(c.param)
	if len(value) == 0 {
		return nil, errors.New(fmt.Sprintf("No param %v", c.param))
	}
	return value, nil
}

type NumParam struct {
	Param
}
func (c NumParam) Get(r *http.Request) (value interface{}, err error) {
	value, err = c.Param.Get(r)
	if err != nil {
		return
	}
	value, err = strconv.Atoi(value.(string))
	if err != nil {
		return nil, errors.New(fmt.Sprintf("Not a number: %v", value))
	}
	return
}

type MapParam struct {
	f func(interface{}) interface{}
	Param
}
func (c MapParam) Get(r *http.Request) (value interface{}, err error) {
	value, err = c.Param.Get(r)
	if err != nil {
		return
	}
	return c.f(value), nil
}
