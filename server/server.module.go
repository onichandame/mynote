package server

import (
	"github.com/onichandame/gim"
	gimgin "github.com/onichandame/gim-gin"
	gimgraphql "github.com/onichandame/gim-graphql"
	"github.com/onichandame/mynote/auth"
	"github.com/onichandame/mynote/note"
	"github.com/onichandame/mynote/ui"
)

var ServerModule = gim.Module{
	Name: `Server`,
	Imports: []*gim.Module{
		&gimgin.GinModule,
		&gimgraphql.GraphqlModule,
		&auth.AuthModule,
		&ui.UIModule,
		&note.NoteModule,
	},
	Providers: []interface{}{newServer, newController},
	Exports:   []interface{}{newServer},
}
