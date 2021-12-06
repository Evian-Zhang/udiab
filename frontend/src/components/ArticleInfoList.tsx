import React from 'react';
import { Button, List, Spin } from 'antd';

export enum LoadingStatus {
    Loading,
    ReadyToLoad,
    NothingToLoad
}

interface ArticleInfoListProps<T> {
    // articleInfos is assumed non-empty
    articleInfos: T[],
    renderItem: (item: T) => React.ReactNode,
    loadingStatus: LoadingStatus,
    onLoadMore: ()=>void,
}

function ArticleInfoList<T>(props: ArticleInfoListProps<T>) {
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
                                加载更多
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
                    {props.renderItem(item)}
                </List.Item>
            )}
            loadMore={loadMore}
        />
    );
}

export default ArticleInfoList;
