package model

import (
	"strconv"

	goutils "github.com/onichandame/go-utils"
	"gorm.io/gorm"
)

type Pagination struct {
	First uint   `graphql:"first,nullable"`
	After string `graphql:"after,nullable"`
	Order string `graphql:"orderBy,nullable"`
}

func (p *Pagination) Paginate(preload int) func(*gorm.DB) *gorm.DB {
	return func(d *gorm.DB) *gorm.DB {
		if p.After != "" {
			offset, err := strconv.Atoi(p.After)
			goutils.Assert(err)
			offset += preload
			d = d.Offset(offset)
		}
		if p.First == 0 {
			d = d.Limit(int(p.First))
		}
		if p.Order != "" {
			d = d.Order(p.Order)
		}
		return d
	}
}
