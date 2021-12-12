package note

import (
	"github.com/onichandame/mynote/model"
	"gorm.io/gorm"
)

type NoteDTO struct {
	ID uint `graphql:"id"`
	NoteInput
}

func (NoteDTO) GetName() string { return `Note` }

type NoteInput struct {
	Title   string `graphql:"title" json:"title"`
	Content string `graphql:"content" json:"content"`
}

type NoteListArgs struct {
	model.Pagination
	Title   string `graphql:"title"`
	Content string `graphql:"content"`
}

func (args *NoteListArgs) Filter() func(*gorm.DB) *gorm.DB {
	return func(d *gorm.DB) *gorm.DB {
		if args.Title != "" {
			d = d.Where("title LIKE ?", args.Title)
		}
		if args.Content != "" {
			d = d.Where("content LIKE ?", args.Content)
		}
		return d
	}
}

type NoteNode struct {
	Node   NoteDTO `graphql:"node"`
	Cursor string  `graphql:"cursor"`
}

type NoteConnection struct {
	PageInfo model.PageInfo `graphql:"pageInfo"`
	Edges    []NoteNode     `graphql:"edges"`
}
