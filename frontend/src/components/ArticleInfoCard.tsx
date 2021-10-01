import React from 'react';
import { Card } from 'antd';
import { ArticleInfo } from '../util';

interface ArticleInfoCardProps {
    articleInfo: ArticleInfo
}

function ArticleInfoCard(props: ArticleInfoCardProps) {
    return(
        <Card
            title={
                <a href={props.articleInfo.url}>
                    {props.articleInfo.title}
                </a>
            }
            style={{width: "100%"}}
        >

        </Card>
    );
}

export default ArticleInfoCard;
