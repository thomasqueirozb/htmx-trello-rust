function findClosestLi(y, target) {
    if (target.tagName === 'LI') {
        return target;
    }

    while (target) {
        if (target.tagName === 'UL') {
            break;
        }
        target = target.parentElement;
    }

    if (target === null) {
        return null;
    }

    let closestLi = null;
    let closestDistance = Number.MAX_VALUE;

    // Traverse the <ul> children and find the closest <li>
    for (let liElement of target.children) {
        if (liElement.tagName !== 'LI') {
            continue;
        }
        let rect = liElement.getBoundingClientRect()

        // Calculate the vertical distance between the mouse pointer and the middle of the li
        let distance = Math.abs(y - (rect.top + rect.height / 2));

        // Update closestLi if this is closer
        if (distance < closestDistance) {
            closestLi = liElement;
            closestDistance = distance;
        }
    }
    return closestLi;
}

function determinePlacement(event) {
    let y = event.clientY
    let closestLi = findClosestLi(y, event.target);

    if (!closestLi) {
        return null;
    }

    let rect = closestLi.getBoundingClientRect();

    return {
        closestLi: closestLi,
        placeBefore: event.clientY < rect.top + rect.height / 2
    };

}
