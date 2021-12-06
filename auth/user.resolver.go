package auth

import (
	"github.com/graphql-go/graphql"
	"github.com/jinzhu/copier"
	gimgraphql "github.com/onichandame/gim-graphql"
	goutils "github.com/onichandame/go-utils"
	"github.com/onichandame/mynote/model"
	structgraphql "github.com/onichandame/struct-graphql"
	"gorm.io/gorm"
)

type userResolver struct{}

func newUserResolver(pwsvc *passwordService, gqlsvc *gimgraphql.GraphqlService, parser *structgraphql.Parser, db *gorm.DB) *userResolver {
	var rsl userResolver
	gqlsvc.AddMutation("createUser", &graphql.Field{
		Type: parser.ParseOutput(new(UserDTO)),
		Args: parser.ParseArgs(new(UserInputDTO)),
		Resolve: func(p graphql.ResolveParams) (res interface{}, err error) {
			defer goutils.RecoverToErr(&err)
			var args UserInputDTO
			goutils.UnmarshalJSONFromMap(p.Args, &args)
			var user model.User
			goutils.Assert(copier.Copy(&user, &args))
			user.Password = pwsvc.hash(user.Password)
			goutils.Assert(db.Create(&user).Error)
			var out UserDTO
			goutils.Assert(copier.Copy(&out, &user))
			res = &out
			return res, err
		},
	})
	return &rsl
}
