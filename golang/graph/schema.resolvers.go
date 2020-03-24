package graph

// This file will be automatically regenerated based on the schema, any resolver implementations
// will be copied through when generating and any unknown code will be moved to the end.

import (
	"context"
	"main/graph/generated"
	"main/graph/model"
)

func (r *myObjResolver) ValueI32(ctx context.Context, obj *model.MyObj) (int, error) {
	return obj.ValueI32, nil
}

func (r *myObjResolver) ValueList(ctx context.Context, obj *model.MyObj) ([]int, error) {
	return obj.ValueList, nil
}

func (r *myObjResolver) Obj(ctx context.Context, obj *model.MyObj) (*model.MyObj, error) {
	return &model.MyObj{
		ValueI32:  999,
		ValueList: makeArray(),
	}, nil
}

func (r *queryResolver) ValueI32(ctx context.Context) (int, error) {
	return 999, nil
}

func (r *queryResolver) Obj(ctx context.Context) (*model.MyObj, error) {
	return &model.MyObj{
		ValueI32:  999,
		ValueList: makeArray(),
	}, nil
}

// MyObj returns generated.MyObjResolver implementation.
func (r *Resolver) MyObj() generated.MyObjResolver { return &myObjResolver{r} }

// Query returns generated.QueryResolver implementation.
func (r *Resolver) Query() generated.QueryResolver { return &queryResolver{r} }

type myObjResolver struct{ *Resolver }
type queryResolver struct{ *Resolver }

// !!! WARNING !!!
// The code below was going to be deleted when updating resolvers. It has been copied here so you have
// one last chance to move it out of harms way if you want. There are two reasons this happens:
//  - When renaming or deleting a resolver the old code will be put in here. You can safely delete
//    it when you're done.
//  - You have helper methods in this file. Move them out to keep these resolver files clean.
func makeArray() []int {
	array := make([]int, 9)
	for i := 0; i < len(array); i++ {
		array[i] = i
	}
	return array
}
