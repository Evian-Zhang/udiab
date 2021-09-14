import React, { useState } from 'react'
import { Input, List, Switch, Space, Select } from 'antd'
import { fetchKeyHints, isSearchKeyValid, SearchSortBy, AdvanceSearchOptions } from '../util';
import debounce from 'debounce'

const { Search } = Input;

let latestKeyHintsFetchedTimestamp = 0;

interface SearchBarProps {
    isSearching: boolean,
    // key is assumed non-empty
    onSearch: (key: string)=>void,
    advanceSearchOptions: AdvanceSearchOptions,
    onAdvanceSearchOptionsChange: (advanceSearchOptions: AdvanceSearchOptions)=>void,
}

function SearchBar(props: SearchBarProps) {
    const [keyHints, setKeyHints] = useState<string[]>([]);
    const [willUseAdvancedSearch, setWillUseAdvancedSearch] = useState(false);

    const advancedSearchVisiblity = willUseAdvancedSearch ? "visible" : "hidden";

    const debouncedFetchKeyHints = debounce((key: string) => {
        fetchKeyHints(key, Date.now())
            .then(([keyHints, timestamp]) => {
                if (timestamp > latestKeyHintsFetchedTimestamp && !props.isSearching) {
                    latestKeyHintsFetchedTimestamp = timestamp;
                    setKeyHints(keyHints);
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
                    justifyContent: "center"
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
                        {keyHint}
                    </List.Item>
                )}
            />}
        </div>
    );
}

export default SearchBar;