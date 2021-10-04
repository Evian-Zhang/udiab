import React from 'react';
import { Card, Spin, List } from 'antd';
import { TopArticleInfo } from '../util';

export interface TopArticlesListCardProps {
    topArticleInfos: TopArticleInfo[]
}

function TopArticlesListCard(props: TopArticlesListCardProps) {
    return(
        <Card title="Top articles">
            {props.topArticleInfos.length === 0
            ?
            <Spin/>
            :
            <List
                dataSource={props.topArticleInfos}
                renderItem={item => (
                    <List.Item>
                        <a href={item.url}>{item.title}</a>
                    </List.Item>
                )}
            />
            }
        </Card>
    );
}

export default TopArticlesListCard;
