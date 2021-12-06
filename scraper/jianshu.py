import requests
import time
import concurrent.futures
import parsel
import re
import json
import pandas as pd
from bs4 import BeautifulSoup
#work_book = openpyxl.Workbook()
from os import system
import os


cookie = ''

headers = {
    'user-agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.45 Safari/537.36',
    'cookie': cookie
}


l = [27, 31, 28, 29, 30, 32,33]


class JianshuUrl(object):


    def getUrl(self,url):
        
        retry_count = 5
        while retry_count > 0:
            try:
                response = requests.get(url=url, proxies=self.get_proxy(), headers=headers)
                if response.status_code:
                    response = response.json()
                    for res in response:
                        slug = res['slug']
                        url = "https://www.jianshu.com/p/"+slug
                        print(url)
                        self.getData(url)
                        # response1 = self.getData(url)
                        # soup = BeautifulSoup(response1.text, "html.parser")
                        # item = soup.find("section",{"class":"ouzJEz"})
                        # title = item.find("section").find("h1").text
                        # content_l = item.find("article").find_all("p")
                        # content = []
                        # for i,contentl in enumerate(content_l):
                        #     content.append(contentl.text)
                        # code_l = item.find("article").find_all("pre")
                        # code = []
                        # for i,codel in enumerate(code_l):
                        #     code.append(codel.text)
                        # likes = item.find("article").find("div",{"class":"_1kCBjS"}).find("div").find("div").find("span").text-"人点赞"
                        # dic = {
                        #     "title":title,
                        #     "source":"JianShu",
                        #     "url":url,
                        #     "article":content,
                        #     "code":code,
                        #     "likes":likes,
                        #     }
                        # with open("JianShu.txt", 'a', encoding = 'utf-8') as f:
                        #     f.write(json.dumps(dic,ensure_ascii=False)+'\n')
                    return url

            except Exception as e:
                retry_count -= 1
                print(e)
          
    def getData(self,url):
        retry_count = 5
        while retry_count > 0:
            try:
                response = requests.get(url = url, proxies=self.get_proxy(), headers=headers)
                if  response.status_code:
                    soup = BeautifulSoup(response.text, "html.parser")
                    resjson = soup.find("script",{"id":"__NEXT_DATA__"})

                    resjson = resjson.string
                    resjson = json.loads(resjson)
                    #resjson = json.loads(resjson.text)
                    #print(resjson)
                    #print(resjson)
                    date = resjson["props"]["initialState"]["note"]["data"]["last_updated_at"]
                    print('////////')
                    print(date)
                    views = resjson["props"]["initialState"]["note"]["data"]["views_count"]
                    print(views)

                    item = soup.find("section",{"class":"ouvJEz"})
                    #print("item: ",item)
                    title = item.find("h1").text
    
                    #print("title: ",title)
                    content_l = item.find("article").find_all("p")
                    content = []
                    for i,contentl in enumerate(content_l):
                        content.append(contentl.text)
                    #print("content: ",content)
                    code_l = item.find("article").find_all("pre")
                    code = []
                    for i,codel in enumerate(code_l):
                        code.append(codel.text)
                    #date = item.find("div").find("div").find("div").find("div",{"class":"s-dsoj"}).find("time").string
                    #views = item.find("div").find("div").find("div").find("div",{"class":"s-dsoj"}).find("span:nth-child(4)").string
    
    
                    dic = {
                        "title":title,
                        "source":"JianShu",
                        "url":url,
                        "content":content,
                        "code":code,
                        "views":views,
                        "date":date
                        #"likes":likes,
                        }
                    with open("JianShu.txt", 'a', encoding = 'utf-8') as f:
                        f.write(json.dumps(dic,ensure_ascii=False)+'\n')
                    return response
            except Exception as e:
                retry_count -= 1
                print(e)
    
    
    def run(self):
        l = [27, 31, 28, 29, 30, 32,33]
        #l = [27]
        with concurrent.futures.ThreadPoolExecutor(max_workers=10) as exe:
            for nav in l:
                for i in range(1,101):
                    url = f'https://www.jianshu.com/programmers?page={i}&type_id={nav}&count=10'
                    exe.submit(self.getUrl,url)       

                    
  
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
    jianshu = JianshuUrl()
    jianshu.run()