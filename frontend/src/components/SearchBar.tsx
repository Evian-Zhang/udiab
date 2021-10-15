import React, { useState } from 'react'
import { Input, List, Switch, Space, Select, message } from 'antd'
import { fetchKeyHints, isSearchKeyValid, SearchSortBy, AdvanceSearchOptions, SearchField, Snippet, BackendRange } from '../util';
import debounce from 'debounce'

const { Search } = Input;

function HighlightText(props: { text: string }) {
    return(
        <b>
            {props.text}
        </b>
    );
}

function composeSnippetHighlight(snippet: string, highlightPositions: BackendRange[]) {
    const encoder = new TextEncoder();
    const decoder = new TextDecoder();
    const encoded_snippet = encoder.encode(snippet);
    let textList = [];
    let unhighlightedStartIndex = 0;
    // according to <https://developer.mozilla.org/en-US/docs/web/javascript/reference/global_objects/string/slice>
    // > If `beginIndex` is greater than or equal to `str.length`, an empty string is returned.
    // > If `endIndex` is greater than `str.length`, `slice()` also extracts to the end of the string.
    for (const highlightPosition of highlightPositions) {
        const unhighlightedSegment = encoded_snippet.slice(unhighlightedStartIndex, highlightPosition.start);
        if (unhighlightedSegment.length > 0) {
            textList.push(decoder.decode(unhighlightedSegment));
        }
        const highlightedSegment = encoded_snippet.slice(highlightPosition.start, highlightPosition.end);
        if (unhighlightedSegment.length > 0) {
            const highlightedComponent = <HighlightText text={decoder.decode(highlightedSegment)} key={`${highlightPosition.start}-${snippet.slice(0, 16)}`}/>;
            textList.push(highlightedComponent);
        }
        unhighlightedStartIndex = highlightPosition.end;
    }
    const unhighlightedSegment = encoded_snippet.slice(unhighlightedStartIndex);
    if (unhighlightedSegment.length > 0) {
        textList.push(decoder.decode(unhighlightedSegment));
    }
    return (
        <>{textList}</>
    );
}

let latestKeyHintsFetchedTimestamp = 0;

interface SearchBarProps {
    isSearching: boolean,
    // key is assumed non-empty
    onSearch: (key: string)=>void,
    advanceSearchOptions: AdvanceSearchOptions,
    onAdvanceSearchOptionsChange: (advanceSearchOptions: AdvanceSearchOptions)=>void,
}

function SearchBar(props: SearchBarProps) {
    const [keyHints, setKeyHints] = useState<Snippet[]>([]);
    const [willUseAdvancedSearch, setWillUseAdvancedSearch] = useState(false);

    const advancedSearchVisiblity = willUseAdvancedSearch ? "visible" : "hidden";

    const debouncedFetchKeyHints = debounce((key: string) => {
        fetchKeyHints(key, Date.now())
            .then(([keyHints, timestamp]) => {
                if (timestamp > latestKeyHintsFetchedTimestamp && !props.isSearching) {
                    latestKeyHintsFetchedTimestamp = timestamp;
                    setKeyHints(keyHints);
                }
            })
            .catch(error => {
                if (error instanceof Error) {
                  message.error(error.message);
                } else {
                  message.error("Unknown error");
                }
            });
    }, 1000);

    function onSearchKeyChange(e: React.ChangeEvent<HTMLInputElement>) {
        if (props.isSearching) {
            return;
        }
        const newSearchKey = e.target.value;
        if (isSearchKeyValid(newSearchKey)) {
            debouncedFetchKeyHints(newSearchKey);
        }
    }

    return (
        <div>
            <div style={{width: "100%"}}>
                <Space>
                    <Switch
                        defaultChecked={willUseAdvancedSearch}
                        onChange={useAdvanceSearch => {
                            setWillUseAdvancedSearch(useAdvanceSearch);
                        }}
                    />
                    <span>
                        Advanced search
                    </span>
                </Space>
                <div
                style={{
                    width: "100%",
                    visibility: advancedSearchVisiblity,
                    display: "flex",
                    justifyContent: "space-around"
                }}>
                    <div>
                        Sort by
                        <Select
                            defaultValue={AdvanceSearchOptions.default().sortBy}
                            style={{width: "auto"}}
                            options={[
                                {
                                    label: "Time",
                                    value: SearchSortBy.Time
                                },
                                {
                                    label: "Hot",
                                    value: SearchSortBy.Hot
                                },
                                {
                                    label: "Relevance",
                                    value: SearchSortBy.Relevance
                                }
                            ]}
                            onChange={(value, _) => {
                                props.onAdvanceSearchOptionsChange({
                                    ...props.advanceSearchOptions,
                                    sortBy: value
                                });
                            }}
                        />
                    </div>
                    <div>
                        Searching
                        <Select
                            defaultValue={AdvanceSearchOptions.default().searchField}
                            style={{width: "auto"}}
                            options={[
                                {
                                    label: "Title",
                                    value: SearchField.Title
                                },
                                {
                                    label: "Code",
                                    value: SearchField.Code
                                },
                                {
                                    label: "All",
                                    value: SearchField.All
                                }
                            ]}
                            onChange={(value, _) => {
                                props.onAdvanceSearchOptionsChange({
                                    ...props.advanceSearchOptions,
                                    searchField: value
                                });
                            }}
                        />
                    </div>
                    <div>
                        <Switch
                            defaultChecked={AdvanceSearchOptions.default().useComplexSearch}
                            onChange={useComplexSearch => {
                                props.onAdvanceSearchOptionsChange({
                                    ...props.advanceSearchOptions,
                                    useComplexSearch
                                });
                            }}
                        />
                        Use Complex Search
                    </div>
                </div>
            </div>
            <Search
                placeholder="Input"
                enterButton
                allowClear
                onChange={onSearchKeyChange}
                loading={props.isSearching}
                onSearch={(value, _) => value.length !== 0 && props.onSearch(value)}
            />
            {keyHints.length !== 0 && !props.isSearching &&
            <List
                bordered
                dataSource={keyHints}
                renderItem={keyHint =>(
                    <List.Item>
                        {composeSnippetHighlight(keyHint.fragments, keyHint.highlightedPositions)}
                    </List.Item>
                )}
            />}
        </div>
    );
}

export default SearchBar;
