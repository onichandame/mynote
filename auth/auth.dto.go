package auth

type LoginInputDTO struct {
	Name     string `graphql:"name"`
	Password string `graphql:"password"`
}

func (LoginInputDTO) GetName() string { return `LoginInput` }
