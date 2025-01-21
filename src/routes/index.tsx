import type { JSX } from 'react';
import { Fragment } from 'react';
import type { TuonoProps } from 'tuono';

interface User {
    name: string,
    email: string,
    vendor: string,
}

interface IndexProps {
	results: User[]
}



export default function IndexPage({data}: TuonoProps<IndexProps>): JSX.Element {
	if (!data?.results) {
		return (
			<Fragment>
				Loading...
			</Fragment>
		);
	}
	return (
		<Fragment>
			Number of Users are: {data.results.length}
			<ul>
				{data.results.map((user, index) => (
					<li key={index}>
						{user.name} - {user.email} - {user.vendor}
					</li>
				))}
			</ul>			
		</Fragment>
	);
}
