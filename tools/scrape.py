import requests
import bs4
from markdownify import markdownify
from pathlib import Path


def main():
    year = int(input("Enter the year: "))
    day = int(input("Enter the day: "))
    cookie = input("Enter your session cookie: ")

    scrape(
        year,
        day,
        cookie,
    )


def scrape(year: int, day: int, cookie: str):
    base_url = f"https://adventofcode.com/{year}/day/{day}"
    input_url = base_url + "/input"
    desc_file = Path(f"../src/descriptions/day_{day:02}.md")
    input_file = Path(f"../src/test_files/day_{day:02}.txt")

    base = requests.get(base_url, cookies={"session": cookie})
    soup = bs4.BeautifulSoup(base.content, "html.parser")
    articles = soup.find_all("article")
    txt = "\n".join([a.decode_contents() for a in articles])
    desc_file.write_text(markdownify(txt))

    input_html = requests.get(input_url, cookies={"session": cookie})
    input_file.write_text(input_html.content.decode("utf-8"))


if __name__ == "__main__":
    main()
