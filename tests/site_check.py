from pathlib import Path
from html.parser import HTMLParser
class Check(HTMLParser):
    def __init__(self): super().__init__(); self.ids=set(); self.links=[]
    def handle_starttag(self, tag, attrs):
        a=dict(attrs)
        if 'id' in a: self.ids.add(a['id'])
        if tag=='a' and 'href' in a: self.links.append(a['href'])
p=Path('index.html'); html=p.read_text(); parser=Check(); parser.feed(html)
assert 'Senior AI and Systems Engineer' in html
assert all(s in html for s in ['Poseidon','Survey Creator','Survey Bank','Audience attributes'])
assert all(x in parser.ids for x in ['work','about','skills','contact'])
assert all(link[1:] in parser.ids for link in parser.links if link.startswith('#') and len(link)>1)
assert '{{PROJECTS}}' not in html
print('Site content and anchor checks passed')
