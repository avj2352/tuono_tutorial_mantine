import type { JSX } from 'react';
import { Fragment } from 'react';
import type { TuonoProps } from 'tuono';

interface IndexProps {
	subtitle: string
}

export default function IndexPage({data}: TuonoProps<IndexProps>): JSX.Element {

	return (
		<Fragment>
			Index Page
		</Fragment>
	);
}
