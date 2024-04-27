package main

import (
	"github.com/KevinZonda/GoX/pkg/iox"
	"os"
	"testing"
)

const README = `
BEGIN
<!-- BEGIN LGBT-CN SIGNATURE -->
User1
User2

User5
<!-- END LGBT-CN SIGNATURE -->
<!-- BEGIN LGBT-CN COUNT -->
已有 -1 人签署！
<!-- END LGBT-CN COUNT -->
END
`

const EXPECTED = `BEGIN
<!-- BEGIN LGBT-CN SIGNATURE -->
User1
User2
User5
<!-- END LGBT-CN SIGNATURE -->
<!-- BEGIN LGBT-CN COUNT -->
已有 3 人签署！
<!-- END LGBT-CN COUNT -->
END`

const TEST_FILE = "test.md"

func TestMainFunc(t *testing.T) {
	defer os.Remove(TEST_FILE)
	iox.WriteAllText(TEST_FILE, README)
	os.Args = []string{"main", TEST_FILE}
	main()
	txt, _ := iox.ReadAllText(TEST_FILE)
	if txt != EXPECTED {
		t.Errorf("Expected %s, got %s", EXPECTED, txt)
		return
	}

}
