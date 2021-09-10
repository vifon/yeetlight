package api

import (
	"errors"
	"fmt"
	"net/http"
	"strconv"
)

// Param is a parameter for the CallMethod calls, usually extracting
// some data from the associated HTTP request provided to the
// Get method.
type Param interface {
	Get(r *http.Request) (interface{}, error)
}

// ConstParam has a constant value completely ignoring the HTTP
// request's contents.
type ConstParam struct {
	Value interface{}
}
func (p ConstParam) Get(*http.Request) (interface{}, error) {
	return p.Value, nil
}

// QueryParam extracts a value from the "param" query parameter.
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

// NumParam is a composite Param that converts the value of the
// original Param to a number.
type NumParam struct {
	Param
	Base int
	BitSize int
}
// NewNumParam creates a NumParam with base 10.
func NewNumParam(p Param) NumParam {
	return NumParam{p, 0, 0}
}
// NewNumParamWithBase creates a NumParam with a custom base.
func NewNumParamWithBase(p Param, base int) NumParam {
	return NumParam{p, base, 0}
}
func (p NumParam) Get(r *http.Request) (value interface{}, err error) {
	origValue, err := p.Param.Get(r)
	if err != nil {
		return
	}
	value, err = strconv.ParseInt(origValue.(string), p.Base, p.BitSize)
	if err != nil {
		return nil, errors.New(fmt.Sprintf("Not a number: %v", origValue))
	}
	return
}

// MapParam is a composite Param that maps the original Param's value
// through an arbitrary unary function.
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
