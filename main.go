/*
 * Copyright (c) 2024. LGBT-CN & KevinZonda
 * This file is part of LGBT-CN Signature Counting.
 */

package main

import (
	"fmt"
	"github.com/KevinZonda/GoX/pkg/iox"
	"github.com/KevinZonda/GoX/pkg/panicx"
	"github.com/KevinZonda/GoX/pkg/stringx"
	"os"
	"strings"
)

const SIGN_BEGIN = "<!-- BEGIN LGBT-CN SIGNATURE -->"
const SIGN_END = "<!-- END LGBT-CN SIGNATURE -->"
const COUNT_BEGIN = "<!-- BEGIN LGBT-CN COUNT -->"
const COUNT_END = "<!-- END LGBT-CN COUNT -->"

func main() {
	fileName := os.Args[1]
	txt, err := iox.ReadAllText(fileName)
	panicx.PanicIfNotNil(err, err)
	count, txt := GetSigCount(txt)
	txt = SetSigCount(txt, count)
	err = iox.WriteAllText(fileName, txt)

}

func Split(content string, beginToken string, endToken string) (header string, body string, footer string) {
	begins := strings.SplitN(content, beginToken, 2)
	header = strings.TrimSpace(begins[0])

	content = begins[1]

	ends := strings.SplitN(content, endToken, 2)
	body = strings.TrimSpace(ends[0])
	footer = strings.TrimSpace(ends[1])
	return

}

func GetSigCount(txt string) (int, string) {
	header, content, footer := Split(txt, SIGN_BEGIN, SIGN_END)
	bodyLines := strings.Split(content, "\n")
	bodyLines = stringx.TrimAllAndClean(bodyLines)
	content = strings.Join(bodyLines, "\n")
	txt = strings.Join([]string{
		header,
		SIGN_BEGIN,
		content,
		SIGN_END,
		footer,
	}, "\n")
	return len(bodyLines), txt
}

func SetSigCount(txt string, count int) string {
	header, _, footer := Split(txt, COUNT_BEGIN, COUNT_END)
	return strings.Join(
		[]string{
			header,
			COUNT_BEGIN,
			fmt.Sprintf("已有 %d 人签署！", count),
			COUNT_END,
			footer,
		}, "\n")
}
