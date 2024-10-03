tidy:
	go mod tidy

lint:
	golangci-lint run

lint_fix:
	golangci-lint run --fix

build:
	go build -o boggle-go

test:
	go test -timeout 5m -cover -covermode=count ./...

typos:
	typos
