package server

import (
	"github.com/onichandame/gim"
	gimgin "github.com/onichandame/gim-gin"
	gimgingql "github.com/onichandame/gim-gingql"
	"github.com/onichandame/mynote/auth"
	"github.com/onichandame/mynote/ui"
)

var ServerModule = gim.Module{
	Name: `Server`,
	Imports: []*gim.Module{gimgingql.NewGinGqlModule(gimgingql.Config{
		Endpoint: `graphql`,
		UseWS:    true,
		Name:     `API`,
		Imports:  []*gim.Module{&auth.AuthModule},
	}), &ui.UIModule, &gimgin.GinModule},
	Providers: []interface{}{newServer},
	Exports:   []interface{}{newServer},
}
