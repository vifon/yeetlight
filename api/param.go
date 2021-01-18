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
func (p ConstParam) Get(*http.Request) (interface{}, error) {
	return p.Value, nil
}

type QueryParam struct {
	param string
}
func (p QueryParam) Get(r *http.Request) (interface{}, error) {
	value := r.URL.Query().Get(p.param)
	if len(value) == 0 {
		return nil, errors.New(fmt.Sprintf("No param %v", p.param))
	}
	return value, nil
}

type NumParam struct {
	Param
}
func (p NumParam) Get(r *http.Request) (value interface{}, err error) {
	origValue, err := p.Param.Get(r)
	if err != nil {
		return
	}
	value, err = strconv.Atoi(origValue.(string))
	if err != nil {
		return nil, errors.New(fmt.Sprintf("Not a number: %v", origValue))
	}
	return
}

type MapParam struct {
	f func(interface{}) interface{}
	Param
}
func (p MapParam) Get(r *http.Request) (value interface{}, err error) {
	value, err = p.Param.Get(r)
	if err != nil {
		return
	}
	return p.f(value), nil
}
