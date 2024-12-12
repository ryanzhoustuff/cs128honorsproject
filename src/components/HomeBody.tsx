import './HomeBody.css'

const HomeBody = () => {
    return (
        <body>
            <div className="home"
                style={{ padding: "" }}
            >
                <h2><mark>Welcome to Honors Poker!</mark></h2>
                <p><mark>Thanks for visiting our website!</mark></p>
                <a href="game" className="button">Play Now</a>
            </div>
        </body>
    )
}

export default HomeBody