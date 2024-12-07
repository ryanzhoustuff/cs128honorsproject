/*import '../style.css'*/
import './HomeBody.css'

const HomeBody = () => {
    return (
        <body>
            <div className="home"
                style={{ padding: "" }}
            >
                <h2>Welcome to Honors Poker!</h2>
                <p>Thanks for visiting our website! Click the button below to start playing!</p>
                <a href="game" className="button">Play Now</a>
            </div>
        </body>
    )
}

export default HomeBody