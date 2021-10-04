import React from 'react';
import { Card, Divider } from 'antd';
import { LikeTwoTone, ClockCircleTwoTone } from '@ant-design/icons';
import { SearchedArticleInfo, BackendRange } from '../util';


function HighlightText(props: { text: string }) {
    return(
        <mark style={{color: 'red'}}>
            {props.text}
        </mark>
    );
}

function composeSnippetHighlight(snippet: string, highlightPositions: BackendRange[]) {
    let textList = [];
    let unhighlightedStartIndex = 0;
    // according to <https://developer.mozilla.org/en-US/docs/web/javascript/reference/global_objects/string/slice>
    // > If `beginIndex` is greater than or equal to `str.length`, an empty string is returned.
    // > If `endIndex` is greater than `str.length`, `slice()` also extracts to the end of the string.
    for (const highlightPosition of highlightPositions) {
        const unhighlightedSegment = snippet.slice(unhighlightedStartIndex, highlightPosition.start);
        if (unhighlightedSegment.length > 0) {
            textList.push(unhighlightedSegment);
        }
        const highlightedSegment = snippet.slice(highlightPosition.start, highlightPosition.end);
        if (unhighlightedSegment.length > 0) {
            const highlightedComponent = <HighlightText text={highlightedSegment} key={`${highlightPosition.start}-${snippet.slice(0, 16)}`}/>;
            textList.push(highlightedComponent);
        }
        unhighlightedStartIndex = highlightPosition.end;
    }
    const unhighlightedSegment = snippet.slice(unhighlightedStartIndex);
    if (unhighlightedSegment.length > 0) {
        textList.push(unhighlightedSegment);
    }
    return (
        <>{textList}</>
    );
}

interface ArticleInfoCardProps {
    articleInfo: SearchedArticleInfo
}

function ArticleInfoCard(props: ArticleInfoCardProps) {
    return(
        <a href={props.articleInfo.url}>
            <Card
                hoverable
                title={composeSnippetHighlight(props.articleInfo.titleSnippet, props.articleInfo.titleHighlightedPositions)}
                style={{width: "100%"}}
            >
                <div style={{width: "100%"}}>
                    {composeSnippetHighlight(props.articleInfo.bodySnippet, props.articleInfo.bodyHighlightedPositions)}
                </div>
                {
                    props.articleInfo.codeSnippet.length !== 0 &&
                    <>
                        <Divider/>
                        <Card type="inner" title="Code snippet">
                            <pre><code>
                                {composeSnippetHighlight(props.articleInfo.codeSnippet, props.articleInfo.codeHighlightedPosition)}
                            </code></pre>
                        </Card>
                    </>
                }
                <Divider/>
                <div style={{width: "100%"}}>
                    <div style={{float: 'left'}}>
                        <LikeTwoTone />
                        {props.articleInfo.likes}
                    </div>
                    <div style={{float: 'right'}}>
                        <ClockCircleTwoTone />
                        {(new Date(props.articleInfo.time)).toLocaleString()}
                    </div>
                </div>
            </Card>
        </a>
    );
}

export default ArticleInfoCard;
