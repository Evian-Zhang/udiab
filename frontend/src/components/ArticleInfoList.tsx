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
            dataSource={props.articleInfos}
            renderItem={item => (
                <ArticleInfoCard
                    articleInfo={item}
                />
            )}
            loadMore={loadMore}
        />
    );
}

export default ArticleInfoList;
