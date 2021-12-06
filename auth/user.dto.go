package auth

type UserDTO struct {
	ID   uint   `graphql:"id"`
	Name string `graphql:"name"`
}

func (UserDTO) GetName() string { return `User` }

type UserInputDTO struct {
	Name     string `graphql:"name" json:"name"`
	Password string `graphql:"password" json:"password"`
}
