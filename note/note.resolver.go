package note

import (
	"fmt"
	"strconv"

	"github.com/graphql-go/graphql"
	"github.com/jinzhu/copier"
	gimgraphql "github.com/onichandame/gim-graphql"
	goutils "github.com/onichandame/go-utils"
	"github.com/onichandame/mynote/auth"
	"github.com/onichandame/mynote/model"
	structgraphql "github.com/onichandame/struct-graphql"
	"gorm.io/gorm"
)

type noteResolver struct{}

func newResolver(db *gorm.DB, gqlsvc *gimgraphql.GraphqlService, parser *structgraphql.Parser) *noteResolver {
	var rsl noteResolver
	gqlsvc.AddQuery(`notes`, &graphql.Field{
		Type: graphql.NewNonNull(parser.ParseOutput(new(NoteConnection))),
		Args: parser.ParseArgs(new(NoteListArgs)),
		Resolve: func(p graphql.ResolveParams) (res interface{}, err error) {
			defer goutils.RecoverToErr(&err)
			var args NoteListArgs
			goutils.UnmarshalJSONFromMap(p.Args, &args)
			var notes []model.Note
			goutils.Assert(db.Scopes(auth.Self(p.Context), args.Filter(), args.Paginate(1)).Find(&notes).Error)
			var preloaded bool
			if args.First > 0 {
				if len(notes) > int(args.First) {
					notes = notes[:len(notes)-1]
					preloaded = true
				}
			}
			var out NoteConnection
			if len(notes) > 0 {
				out.PageInfo.HasNextPage = preloaded
				out.PageInfo.HasPreviousPage = args.After != "" && args.After != "0"
				after, err := strconv.Atoi(args.After)
				goutils.Assert(err)
				out.PageInfo.StartCursor = fmt.Sprintf("%v", after+1)
				out.PageInfo.EndCursor = fmt.Sprintf("%v", after+len(notes))
				out.Edges = make([]NoteNode, 0)
				for i, note := range notes {
					var dto NoteDTO
					goutils.Assert(copier.Copy(&dto, &note))
					out.Edges = append(out.Edges, NoteNode{Node: dto, Cursor: fmt.Sprintf("%v", after+i+1)})
				}
			}
			res = &out
			return res, err
		},
	})
	gqlsvc.AddQuery(`note`, &graphql.Field{
		Type: graphql.NewNonNull(parser.ParseOutput(new(NoteDTO))),
		Args: graphql.FieldConfigArgument{
			"id": &graphql.ArgumentConfig{
				Type: graphql.NewNonNull(graphql.String),
			},
		},
		Resolve: func(p graphql.ResolveParams) (res interface{}, err error) {
			defer goutils.RecoverToErr(&err)
			id := p.Args[`id`].(string)
			var user model.User
			goutils.Assert(db.Scopes(auth.Self(p.Context)).First(&user, id).Error)
			var out NoteDTO
			goutils.Assert(copier.Copy(&out, &user))
			res = &out
			return res, err
		},
	})
	gqlsvc.AddMutation(`createNote`, &graphql.Field{
		Type: graphql.NewNonNull(parser.ParseOutput(new(NoteDTO))),
		Args: parser.ParseArgs(new(NoteInput)),
		Resolve: func(p graphql.ResolveParams) (res interface{}, err error) {
			defer goutils.RecoverToErr(&err)
			var input NoteInput
			goutils.UnmarshalJSONFromMap(p.Args, &input)
			goutils.Assert(db.Scopes(auth.Self(p.Context)).Error)
			return res, err
		},
	})
	gqlsvc.AddMutation(`updateNote`, &graphql.Field{
		Type: graphql.NewNonNull(parser.ParseOutput(new(NoteDTO))),
	})
	return &rsl
}
