package main

import (
	"context"
	"fmt"
	"github.com/99designs/gqlgen/graphql"
	"github.com/99designs/gqlgen/graphql/executor"
	"main/graph"
	"main/graph/generated"
	"sync"
	"time"
)

func main() {
	c := generated.Config{Resolvers: &graph.Resolver{}}
	schema := generated.NewExecutableSchema(c)

	s := time.Now()

	var wg sync.WaitGroup
	wg.Add(4)
	for i := 0; i < 4; i++ {
		go func() {
			for i := 0; i < 100000; i++ {
				e := executor.New(schema)
				operationCtx, _ := e.CreateOperationContext(graphql.StartOperationTrace(context.Background()), &graphql.RawParams{
					Query: `
            {
                            valueI32 obj {
                                valueI32 valueList obj {
                                    valueI32 valueList obj {
                                        valueI32 valueList obj {
                                            valueI32 valueList obj {
                                                valueI32 valueList obj {
                                                    valueI32 valueList obj {
                                                        valueI32 valueList obj {
                                                            valueI32 valueList obj {
                                                                valueI32 valueList obj {
                                                                    valueI32 valueList obj {
                                                                        valueI32 valueList obj {
                                                                            valueI32 valueList obj {
                                                                                valueI32 valueList obj {
                                                                                    valueI32 valueList obj {
                                                                                        valueI32 valueList
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
`,
					OperationName: "",
					Variables:     nil,
					Extensions:    nil,
					ReadTime:      graphql.TraceTiming{},
				})
				exec := schema.Exec(graphql.WithOperationContext(context.Background(), operationCtx))
				exec(context.Background())
			}

			wg.Done()
		}()
	}

	wg.Wait()

	fmt.Printf("gqlgen: %d ms\n", time.Now().Sub(s).Milliseconds())
}
