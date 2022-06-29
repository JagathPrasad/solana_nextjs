import React from 'react';
import styles from '../styles/uploadmodal.module.css';

const UploadModal = ({
    description
    , videoUrl
    , newVideo
    , setDescription
    , setVideoUrl
    , setNewVideoShow
}) => {

    return (
        <div className={styles.wrapper}>
            <div className={styles.title}>Upload new video</div>
            <div className={styles.inputField}>
                <div className={styles.inputTitle}>Descriptoin</div>
                <div className={styles.inputContainer}>
                    <input type="text" className={style.input} value={description} onChange={e => setDescription(e.target.value)} />

                </div>
            </div>
            <div className={styles.inputField}>
                <div className={styles.inputTitle}>Descriptoin</div>
                <div className={styles.inputContainer}>
                    <input type="text" className={style.input} value={videoUrl} onChange={e => setVideoUrl(e.target.value)} />

                </div>
            </div>
            <div className={styles.modalButtons}>
                <button onClick={() => setNewVideoShow(false)} className={`${styles.button} ${styles.cancelButton}`}>cancel</button>
                <button onClick={() => newVideo} className={`${styles.button} ${styles.createButton}`}>save</button>
            </div>
        </div>
    )
}
export default UploadModal;