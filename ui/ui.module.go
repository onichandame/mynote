package ui

import (
	"github.com/onichandame/gim"
	gimgin "github.com/onichandame/gim-gin"
)

var UIModule = gim.Module{
	Name:      `UI`,
	Imports:   []*gim.Module{&gimgin.GinModule},
	Providers: []interface{}{newUI, newUIController},
}
