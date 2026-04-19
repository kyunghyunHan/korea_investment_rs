#!/usr/bin/env python3

import argparse
import json
import zipfile
import xml.etree.ElementTree as ET
from collections import Counter
from pathlib import Path


NS = {
    "a": "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
    "r": "http://schemas.openxmlformats.org/officeDocument/2006/relationships",
    "pr": "http://schemas.openxmlformats.org/package/2006/relationships",
}


def read_shared_strings(archive: zipfile.ZipFile) -> list[str]:
    root = ET.fromstring(archive.read("xl/sharedStrings.xml"))
    values: list[str] = []
    for item in root.findall("a:si", NS):
        values.append("".join(node.text or "" for node in item.iterfind(".//a:t", NS)))
    return values


def read_workbook_mapping(archive: zipfile.ZipFile) -> dict[str, str]:
    rel_root = ET.fromstring(archive.read("xl/_rels/workbook.xml.rels"))
    rel_map = {
        rel.attrib["Id"]: rel.attrib["Target"]
        for rel in rel_root.findall("pr:Relationship", NS)
    }

    workbook = ET.fromstring(archive.read("xl/workbook.xml"))
    sheet_map: dict[str, str] = {}
    for sheet in workbook.find("a:sheets", NS):
        rel_id = sheet.attrib[
            "{http://schemas.openxmlformats.org/officeDocument/2006/relationships}id"
        ]
        sheet_map[sheet.attrib["name"]] = rel_map[rel_id]
    return sheet_map


def cell_value(cell: ET.Element, shared_strings: list[str]) -> str:
    value = cell.find("a:v", NS)
    if value is None:
        return ""

    raw = value.text or ""
    if cell.attrib.get("t") == "s":
        return shared_strings[int(raw)]
    return raw


def extract_rows(xlsx_path: Path, sheet_name: str) -> list[dict[str, str]]:
    with zipfile.ZipFile(xlsx_path) as archive:
        shared_strings = read_shared_strings(archive)
        workbook_mapping = read_workbook_mapping(archive)
        worksheet = ET.fromstring(archive.read(f"xl/{workbook_mapping[sheet_name]}"))
        rows = worksheet.find("a:sheetData", NS)

        extracted: list[dict[str, str]] = []
        for row in list(rows)[1:]:
            cols = {
                "".join(ch for ch in cell.attrib["r"] if ch.isalpha()): cell_value(
                    cell, shared_strings
                )
                for cell in row
            }
            if cols.get("A"):
                extracted.append(cols)
        return extracted


def build_summary(rows: list[dict[str, str]]) -> dict[str, object]:
    return {
        "total": len(rows),
        "by_protocol": dict(Counter(row.get("B", "") for row in rows)),
        "by_http_method": dict(Counter(row.get("H", "") for row in rows)),
        "top_menu_positions": Counter(row.get("C", "") for row in rows).most_common(20),
    }


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Extract the KIS Open API inventory from the xlsx workbook."
    )
    parser.add_argument("xlsx", type=Path, help="Path to the KIS xlsx file")
    parser.add_argument(
        "--sheet",
        default="API 목록",
        help="Worksheet name to parse. Defaults to 'API 목록'.",
    )
    parser.add_argument(
        "--format",
        choices=("json", "markdown"),
        default="json",
        help="Output format.",
    )
    args = parser.parse_args()

    rows = extract_rows(args.xlsx, args.sheet)
    summary = build_summary(rows)

    if args.format == "json":
        print(
            json.dumps(
                {
                    "summary": summary,
                    "rows": rows,
                },
                ensure_ascii=False,
                indent=2,
            )
        )
        return

    print("# KIS API Inventory")
    print()
    print(f"- Total APIs: {summary['total']}")
    print(f"- Protocol split: {summary['by_protocol']}")
    print(f"- HTTP methods: {summary['by_http_method']}")
    print("- Top menu positions:")
    for name, count in summary["top_menu_positions"]:
        print(f"  - {count}: {name}")


if __name__ == "__main__":
    main()
