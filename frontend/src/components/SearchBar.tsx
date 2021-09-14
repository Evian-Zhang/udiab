import React, { useState } from 'react'
import { Input, List } from 'antd'
import { fetchKeyHints } from '../util';
import debounce from 'debounce'

const { Search } = Input;

function SearchBar() {
    const [keyHints, setKeyHints] = useState<string[]>([]);

    function onSearchChange(e: React.ChangeEvent<HTMLInputElement>) {
        const newSearchKey = e.target.value;
        
    }

    return (
        <div>
            <Search
                placeholder="Input"
                enterButton
                allowClear
                onChange={onSearchChange}
            />
            {keyHints.length != 0 &&
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
