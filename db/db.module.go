package db

import "github.com/onichandame/gim"

var DBModule = gim.Module{
	Name:      `DB`,
	Providers: []interface{}{newDB},
	Exports:   []interface{}{newDB},
}
