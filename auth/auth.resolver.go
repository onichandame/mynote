package auth

import (
	"errors"

	"github.com/graphql-go/graphql"
	gimgraphql "github.com/onichandame/gim-graphql"
	goutils "github.com/onichandame/go-utils"
	"github.com/onichandame/mynote/model"
	structgraphql "github.com/onichandame/struct-graphql"
	"gorm.io/gorm"
)

type authResolver struct{}

func newAuthResolver(sesssvc *SessionService, pwsvc *passwordService, db *gorm.DB, gqlsvc *gimgraphql.GraphqlService, parser *structgraphql.Parser) *authResolver {
	var rsl authResolver
	gqlsvc.AddMutation("login", &graphql.Field{
		Type: graphql.NewNonNull(graphql.String),
		Args: parser.ParseArgs(new(LoginInputDTO)),
		Resolve: func(p graphql.ResolveParams) (res interface{}, err error) {
			defer goutils.RecoverToErr(&err)
			var args LoginInputDTO
			goutils.UnmarshalJSONFromMap(p.Args, &args)
			var user model.User
			goutils.Assert(db.First(&user, "name=?", args.Name).Error)
			if !pwsvc.validate(args.Password, user.Password) {
				panic(errors.New("password incorrect"))
			}
			res = sesssvc.create(&user)
			return res, err
		},
	})
	return &rsl
}
