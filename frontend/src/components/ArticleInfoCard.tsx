import React from 'react';
import { Card } from 'antd';
import { SearchedArticleInfo } from '../util';

interface ArticleInfoCardProps {
    articleInfo: SearchedArticleInfo
}

function ArticleInfoCard(props: ArticleInfoCardProps) {
    return(
        <Card
            title={
                <a href={props.articleInfo.url}>
                    {props.articleInfo.titleSnippet}
                </a>
            }
            style={{width: "100%"}}
        >

        </Card>
    );
}

export default ArticleInfoCard;
