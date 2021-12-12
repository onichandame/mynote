package model

import "gorm.io/gorm"

type Note struct {
	gorm.Model
	UserID  uint `gorm:"not null"`
	User    *User
	Title   string `gorm:"not null"`
	Content string `gorm:"not null"`
}
