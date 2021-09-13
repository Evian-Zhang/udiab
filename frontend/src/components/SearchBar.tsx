import React from 'react'
import { Input } from 'antd'

const { Search } = Input;

function SearchBar() {
    return (
        <div>
            <Search
                placeholder="Input"
                enterButton
                allowClear
            />
        </div>
    );
}

export default SearchBar;
