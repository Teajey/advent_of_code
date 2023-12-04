package main

import "testing"

func TestScanForNum(t *testing.T) {
	s, e := scanForNum("....123..", 0)
	if s != 4 {
		t.Error("s != 4:", s)
	}
	if e != 7 {
		t.Error("e != 7:", e)
	}
}

func TestScanForNumEmpty(t *testing.T) {
	s, e := scanForNum("", 0)
	if s != -1 {
		t.Error("s != -1:", s)
	}
	if e != -1 {
		t.Error("e != -1:", e)
	}
}

func TestScanForNumEnd(t *testing.T) {
	s, e := scanForNum(".....45", 0)
	if s != 5 {
		t.Error("s != 5:", s)
	}
	if e != 7 {
		t.Error("e != 7:", e)
	}
}

func TestScanForNumOneDigit(t *testing.T) {
	s, e := scanForNum("....1....", 0)
	if s != 4 {
		t.Error("s != 4:", s)
	}
	if e != 5 {
		t.Error("e != 5:", e)
	}
}

func TestScanForNumEndOffByOne(t *testing.T) {
	s, e := scanForNum(".....45.", 0)
	if s != 5 {
		t.Error("s != 5:", s)
	}
	if e != 7 {
		t.Error("e != 7:", e)
	}
}

func TestScanForNumAfterAnother(t *testing.T) {
	s, e := scanForNum("...661...485..", 6)
	if s != 9 {
		t.Error("s != 9:", s)
	}
	if e != 12 {
		t.Error("e != 12:", e)
	}
}
