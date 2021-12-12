package note

import (
	"github.com/onichandame/gim"
	gimgraphql "github.com/onichandame/gim-graphql"
	"github.com/onichandame/mynote/db"
)

var NoteModule = gim.Module{
	Name:      `Note`,
	Imports:   []*gim.Module{&db.DBModule, &gimgraphql.GraphqlModule},
	Providers: []interface{}{newResolver},
}
