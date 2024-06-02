export default function typewriter(element, properties) {
    let index = 0;
    let inTag = false;
    const interval = () => {
        if (properties.text[index] === "<") {
            inTag = true;
        }
        if (properties.text[index] === ">") {
            inTag = false;
        }
        if (!inTag) {
            document.querySelector(element).innerHTML = properties.text.slice(
                0,
                index + 1
            );
            index++;
            if (index >= properties.text.length) {
                clearInterval(intervalId);
                if (properties.onFinish === undefined) return;
                properties.onFinish();
            }
        } else {
            index++;
            interval();
        }
    };
    let intervalId = setInterval(interval, properties.speed);
}
