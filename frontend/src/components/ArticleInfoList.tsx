import React from 'react';
import { Button, List, Spin } from 'antd';
import { SearchedArticleInfo } from '../util'
import ArticleInfoCard from './ArticleInfoCard'

export enum LoadingStatus {
    Loading,
    ReadyToLoad,
    NothingToLoad
}

interface ArticleInfoListProps {
    // articleInfos is assumed non-empty
    articleInfos: SearchedArticleInfo[],
    loadingStatus: LoadingStatus,
    onLoadMore: ()=>void,
}

function ArticleInfoList(props: ArticleInfoListProps) {
    const loadMore = (
        <div
            style={{
                display: "flex",
                justifyContent: "center"
            }}
        > {
            function() {
                switch (props.loadingStatus) {
                    case LoadingStatus.Loading:
                        return (<Spin/>);
                    case LoadingStatus.ReadyToLoad:
                        return (
                            <Button
                                onClick={props.onLoadMore}
                            >
                                Load more
                            </Button>
                        );
                    case LoadingStatus.NothingToLoad:
                        return null;
                }
            }()
        } </div>
    );

    return (
        <List
            grid={{column: 1, gutter: 16}}
            dataSource={props.articleInfos}
            renderItem={item => (
                <List.Item>
                    <ArticleInfoCard
                        articleInfo={item}
                    />
                </List.Item>
            )}
            loadMore={loadMore}
        />
    );
}

export default ArticleInfoList;
