import requests
import time
import concurrent.futures
import parsel
import re
import json
import pandas as pd
from bs4 import BeautifulSoup
import time
import os

cookie = ''

headers = {
    'user-agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.45 Safari/537.36',
    'cookie': cookie
}


l = ['home', 'career', 'python', 'java', 'c', 'ai', 'web', 'arch', 'blockchain', 'db', '5g', 'game', 'mobile',
     'ops', 'sec', 'cloud', 'engineering', 'iot', 'fund', 'avi', 'other']

class CsdnUrl(object):

    def getData(self,url):
        retry_count = 5
        while retry_count > 0:
            
            try:
                res = requests.get(url=url, headers=headers, proxies=self.get_proxy())
        
                if res.status_code:
                    res = res.json()
                    data = res['articles']
                    for da in data:
                        #title = da['title']
                        url_ = da['url']
                        #sheet.append(arr)
                        #print(url_)
                        self.getData1(url_)
                        # with open('csdn_url.txt',"a",encoding='utf-8') as f:
                        #     f.write(str(url_)+'\n')
        
                    return res
                else:
                    print('检测到被网页反爬！')
            except Exception as e:
                retry_count -= 1
                print(e)
                
    def getData1(self,url):
        retry_count = 5
        while retry_count > 0:
            try:
                response = requests.get(url=url, headers=headers, proxies=self.get_proxy())
                if response.status_code:
                    soup = BeautifulSoup(response.text, "html.parser")
                    item = soup.find("div",{"class": "blog-content-box"})
                    title = item.find("div").find("div").find("div").find("h1").string
                    views = item.find("div").find("div").find("div",{"class": "article-info-box"}).find("div").find("div").find("span",{"class":"read-count"}).string
                    date = item.find("div").find("div").find("div",{"class": "article-info-box"}).find("div").find("div").find("span",{"class":"time"}).string
                    date = time.strptime(date,'%Y-%m-%d %H:%M:%S')
                    date = time.mktime(date)
                    content_l = item.find("article").find("div").find("div").find_all("p")
                    content = []
                    for contentl in content_l:
                        content.append(contentl.text)
                    code_l = item.find("article").find("div").find("div").find_all("pre")
                    code = []
                    for codel in code_l:
                        code.append(codel.text)
                    dic = {
                        "title":title,
                        "source":"CSDN",
                        "url":url,
                        "content":content,
                        "code":code,
                        "views":views,
                        "date":date}
                    with open("CSDN.txt", "a",encoding = 'utf-8') as f:
                        f.write(json.dumps(dic,ensure_ascii=False)+'\n')
                    return response
            except Exception as e:
                retry_count -= 1
                print(e)
    
    def run(self):
        l = ['home', 'career', 'python', 'java', 'c', 'ai', 'web', 'arch', 'blockchain', 'db', '5g', 'game', 'mobile',
             'ops', 'sec', 'cloud', 'engineering', 'iot', 'fund', 'avi', 'other']
        with concurrent.futures.ThreadPoolExecutor(max_workers=4) as exe:
            for nav in l:
                for i in range(500):
                    url = f'https://blog.csdn.net/api/articles?type=more&category={nav}&shown_offset=0'
                    exe.submit(self.getData,url)
                    
    

    
    
    
    def get_proxy(self):
        proxyHost = "dyn.horocn.com"
        proxyPort = "50000"
    
        proxyUser = os.getenv('proxyUser')
        proxyPass = os.getenv('proxyPass')
    
        proxyMeta = "http://%(user)s:%(pass)s@%(host)s:%(port)s" % {
            "host": proxyHost,
            "port": proxyPort,
            "user": proxyUser,
            "pass": proxyPass,
            }
        proxies = {
            "http": proxyMeta,
            "https": proxyMeta,
            }

        return proxies
if __name__ == '__main__':
    csdn = CsdnUrl()
    csdn.run()