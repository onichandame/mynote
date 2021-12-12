package auth

import (
	"github.com/onichandame/gim"
	gimgin "github.com/onichandame/gim-gin"
	gimgraphql "github.com/onichandame/gim-graphql"
	"github.com/onichandame/mynote/db"
)

var AuthModule = gim.Module{
	Name:      `Auth`,
	Imports:   []*gim.Module{&gimgraphql.GraphqlModule, &db.DBModule, &gimgin.GinModule},
	Providers: []interface{}{newUserResolver, newPasswordService, newAuthResolver, newSessionsService, newAuthMiddleware},
	Exports:   []interface{}{newSessionsService},
}
