import React from 'react';
import { useRef, useState } from 'react';
import Comments from './Comments';
import Sidebar from './Sidebar';
import Footer from './Footer';
import styles from './styles/video.module.css'


const Video = ({
    address
    , url
    , channel, index
    , likes
    , description
    , likeVideo
    , likesAddress
    , createComment
    , getComments
    , commentCount
}) => {

    const [playing, setPlaying] = useState(flase);
    const [showCommnetModal, setShowCommentModal] = useState(false);
    const videoRef = useRef(null);
    const onVidoePress = () => {
        if (playing) {
            videoRef.current.pause();
            setPlaying(false)
        }
        else {
            videoRef.current.play();
            setPlaying(true)
        }
    }

    const hideComments = () => {
        setShowCommentModal(false);
    }
    const showComments = () => {
        setShowCommentModal(true);
    }


    return (
        <div className={styles.wrapper}>
            <video className={styles.videoPlayer} loop onClick={onVideoPress} ref={videoRef} src={url} style={{ objectFit: 'cover' }} />
            {showCommnetModal && (
                <Comments />
            )}

            <Footer
                channel={channel}
                description={description}
                song={index}
            />

            <Sidebar
                address={address}
                likes={likes}
                shares={shares}
                onShowComments={showComments}
                likeVideo={likeVideo}
                index={index}
                likesAddress={likesAddress}
                messages={commentCount}
            />
            {showCommnetModal && (
                <Comments
                    onHide={hideComments}
                    index={index}
                    address={address}
                    createComment={createComment}
                    getComments={getComments}
                    commentCount={commentCount}
                />
            )}
        </div>
    )
}
export default Video;