import React from 'react';
import styles from '../styles/bottombar.module.css';

import { AiFillHome, AiOutlineCompass } from 'react-icons/ai';
import { IoIosAdd } from 'react-icons/io';
import { BiMessageMinus } from 'react-icons/bi';
import { BsPerson } from 'react-icons/bs';


const BootmBar = ({
    setNewVideoShow,
    getTiktoks
}) => {

    return (
        <div className={styles.wrapper}>
            <AiFillHome className={styles.bottomIcon} />
            <AiOutlineCompass className={styles.bottomIcon} onClick={() => getTiktoks}></AiOutlineCompass>
            <div className={styles.addVideoButton}>
                <IoIosAdd className={styles.bottomIcon} onClick={() => setNewVideoShow(true)} style={{ color: black }} />

            </div>
            <BiMessageMinus className={styles.bottomIcon} />
            <BsPerson className={styles.bottomIcon} />
        </div>
    )
}
export default BootmBar;