import requests
from bs4 import BeautifulSoup
import pymongo
import re
import concurrent.futures
import time
import json
from urllib import request
import os


class CnBlog(object):
    def __init__(self):
        user_agent = 'Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/50.0.2661.102 Safari/537.36'
        self.headers = {'Cache-Control': 'max-age=0',
                        'Connection': 'keep-alive',
                        'Accept': 'text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8',
                        'User-Agent': user_agent,
                        }

    def getConnect(self):
        con = pymongo.MongoClient('localhost', 27017)
        cnblog = con['cnblog']
        blog_sites = cnblog['sites']
        return blog_sites


    # 获得界面
    def getPage(self, url=None):
        retry_count = 5
        while retry_count > 0:
            try:
                # proxy = {"http": self.get_proxy() + ""}
                response = requests.get(url, proxies=self.get_proxy(), headers=self.headers)
                #response = requests.get(url, headers=self.headers)
                # print(response)
                soup = BeautifulSoup(response.text, "html.parser")
                # print(soup.prettify())
                return soup
            except Exception as e:
                retry_count -= 1
                print(e)

    def parsePage(self, url=None):
      
        # 获取界面

        soup = self.getPage(url)
        itemBlog = soup.find_all("article")
        print(itemBlog.__len__())
        blog = CnBlog()
        for i,blogInfo in enumerate(itemBlog):
            #blog.num = i
            blog.url = blogInfo.find("section").find("div").find("a", {"class": "post-item-title"}).get("href")

            print(blog.url)
            blog.title = blogInfo.find("section").find("div").find("a", {"class": "post-item-title"}).string
            
            blog.date = blogInfo.find("section").find("footer").find("span",{"class":"post-meta-item"}).find("span").string
            
            blog.date = time.strptime(blog.date+":00",'%Y-%m-%d %H:%M:%S')
            blog.date = time.mktime(blog.date)
            print(blog.date)
            blog.view = blogInfo.find("section").find("footer").find("a",{"href":str(blog.url)}).find("span").string
            print(blog.view)

            item = self.getPage(blog.url)
            blog.articles = item.find("div",{"id": "cnblogs_post_body"}).find_all("p")
            blog_article = []
            for i,blogarticle in enumerate(blog.articles):
                blog_article.append(blogarticle.text)
            #print(blog_article)
            blog.codes = item.find("div",{"id": "cnblogs_post_body"}).find_all("pre")
            blog_code = []
            for i,blogcode in enumerate(blog.codes):
                blog_code.append(blogcode.text)
            #print(blog_code)
            print(1)
            
            # blog.view = item.find("div",{"class":"post"}).find("p",{"class":"postfoot"})#.find("span",{"id":"post_view_count"}).string
            # if blog.view == None:
            #     blog.view = item.find("div",{"class":"post"}).find("p",{"class":"postDesc"})
            # blog.view = blog.view.find("span",{"id":"post_view_count"}).string
            # print(blog.view)
            dic = {
                "title":blog.title,
                "source":"CnBlog",
                "url":blog.url,
                "content":blog_article,
                "code":blog_code,
                "views":blog.view,
                "date":blog.date,
            }
            with open('CnBlog.txt','a',encoding='utf-8') as f:
                f.write(json.dumps(dic,ensure_ascii=False)+'\n')
        return 0
        

 


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



    def filter_tags(self,htmlstr):
        # 先过滤CDATA
        re_cdata = re.compile('//<!\[CDATA\[[^>]*//\]\]>', re.I)  # 匹配CDATA
        re_script = re.compile('<\s*script[^>]*>[^<]*<\s*/\s*script\s*>', re.I)  # Script
        re_style = re.compile('<\s*style[^>]*>[^<]*<\s*/\s*style\s*>', re.I)  # style
        re_br = re.compile('<br\s*?/?>')  # 处理换行
        re_h = re.compile('</?\w+[^>]*>')  # HTML标签
        re_comment = re.compile('<!--[^>]*-->')  # HTML注释
        s = re_cdata.sub('', htmlstr)  # 去掉CDATA
        s = re_script.sub('', s)  # 去掉SCRIPT
        s = re_style.sub('', s)  # 去掉style
        s = re_br.sub('\n', s)  # 将br转换为换行
        s = re_h.sub('', s)  # 去掉HTML 标签
        s = re_comment.sub('', s)  # 去掉HTML注释
        # 去掉多余的空行
        blank_line = re.compile('\n+')
        s = blank_line.sub('\n', s)
        s = self.replaceCharEntity(s)  # 替换实体
        return s


    def replaceCharEntity(self,htmlstr):
        CHAR_ENTITIES = {'nbsp': ' ', '160': ' ',
                         'lt': '<', '60': '<',
                         'gt': '>', '62': '>',
                         'amp': '&', '38': '&',
                         'quot': '"', '34': '"', }

        re_charEntity = re.compile(r'&#?(?P<name>\w+);')
        sz = re_charEntity.search(htmlstr)
        while sz:
            try:
                global key
                entity = sz.group()  # entity全称，如&gt;
                key = sz.group('name')  # 去除&;后entity,如&gt;为gt
                htmlstr = re_charEntity.sub(CHAR_ENTITIES[key], htmlstr, 1)
                sz = re_charEntity.search(htmlstr)
            except KeyError:
                # 以空串代替
                htmlstr = re_charEntity.sub('', htmlstr, 1)
                sz = re_charEntity.search(htmlstr)
        return htmlstr

    def repalce(self,s, re_exp, repl_string):
        return re_exp.sub(repl_string, s)


def run():
    with concurrent.futures.ThreadPoolExecutor(max_workers=10) as exe:
        cnblog = CnBlog()
        for i in range(1,201):
            url = f"https://www.cnblogs.com/#p{i}"
            print(url)
            exe.submit(cnblog.parsePage,url)
            # cnblog.parsePage(url)


if __name__ == "__main__":
    run()