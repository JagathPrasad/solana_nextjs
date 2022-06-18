import React from 'react';
import Signup from './Signup';

const MainView = () => {
    const isAccount = true;
    return (
        <>
            {isAccount ? (
                <div>Ticktok yes</div>
            ) : (
                <Signup />)}
        </>
    )
}

export default MainView;