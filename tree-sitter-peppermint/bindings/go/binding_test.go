package tree_sitter_peppermint_test

import (
	"testing"

	tree_sitter "github.com/smacker/go-tree-sitter"
	"github.com/tree-sitter/tree-sitter-peppermint"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_peppermint.Language())
	if language == nil {
		t.Errorf("Error loading Peppermint grammar")
	}
}
