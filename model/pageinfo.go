package model

type PageInfo struct {
	HasPreviousPage bool   `graphql:"hasPreviousPage"`
	HasNextPage     bool   `graphql:"hasNextPage"`
	StartCursor     string `graphql:"startCursor"`
	EndCursor       string `graphql:"endCursor"`
}
